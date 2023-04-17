package kz.ilotterytea.fembajbot.entities;

import jakarta.persistence.*;
import org.hibernate.annotations.CreationTimestamp;
import org.hibernate.annotations.UpdateTimestamp;

import java.util.Date;
import java.util.HashSet;
import java.util.Set;

/**
 * @author ilotterytea
 * @version 1.0
 */
@Entity
@Table(name = "consumers")
public class Consumer {
    @Id
    @GeneratedValue(strategy = GenerationType.IDENTITY)
    private Integer id;

    @Column(name = "alias_id", unique = true, updatable = false, nullable = false)
    private Integer aliasId;

    @Column(name = "alias_name", unique = true, nullable = false)
    private String aliasName;

    @Column(name = "points", nullable = false)
    private Integer points;

    @Column(name = "total_points", nullable = false)
    private Integer totalPoints;

    @CreationTimestamp
    @Temporal(TemporalType.TIMESTAMP)
    @Column(name = "created_at", updatable = false, nullable = false)
    private Date creationTimestamp;

    @UpdateTimestamp
    @Temporal(TemporalType.TIMESTAMP)
    @Column(name = "updated_at", nullable = false)
    private Date updateTimestamp;

    @OneToMany(mappedBy = "consumer", cascade = CascadeType.MERGE, fetch = FetchType.EAGER)
    private Set<Action> actions;

    public Consumer(Integer aliasId, String aliasName) {
        this.aliasId = aliasId;
        this.aliasName = aliasName;
        this.points = 0;
        this.totalPoints = 0;
        this.actions = new HashSet<>();
    }

    public Consumer() {}

    public Integer getId() {
        return id;
    }

    public void setId(Integer id) {
        this.id = id;
    }

    public Integer getAliasId() {
        return aliasId;
    }

    public void setAliasId(Integer aliasId) {
        this.aliasId = aliasId;
    }

    public String getAliasName() {
        return aliasName;
    }

    public void setAliasName(String aliasName) {
        this.aliasName = aliasName;
    }

    public Integer getPoints() {
        return points;
    }

    public void setPoints(Integer points) {
        this.points = points;
    }

    public Integer getTotalPoints() {
        return totalPoints;
    }

    public void setTotalPoints(Integer totalPoints) {
        this.totalPoints = totalPoints;
    }

    public Date getCreationTimestamp() {
        return creationTimestamp;
    }

    public void setCreationTimestamp(Date creationTimestamp) {
        this.creationTimestamp = creationTimestamp;
    }

    public Date getUpdateTimestamp() {
        return updateTimestamp;
    }

    public void setUpdateTimestamp(Date updateTimestamp) {
        this.updateTimestamp = updateTimestamp;
    }

    public Set<Action> getActions() {
        return actions;
    }

    public void setActions(Set<Action> actions) {
        for (Action action : actions) {
            action.setConsumer(this);
        }
        this.actions = actions;
    }

    public void addAction(Action action) {
        action.setConsumer(this);
        this.actions.add(action);
    }

    public void removeAction(Action action) {
        this.actions.remove(action);
    }
}
